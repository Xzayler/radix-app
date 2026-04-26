'use server';
import { db } from './db';
import {
  usersTable,
  systemsTable,
  digitsTable,
  jobsTable,
  favouritesTable,
} from './schema';
import {
  FavouriteDbInsert,
  JobDbInsert,
  SystemDbInsert,
  UserDbEntity,
  UserDbInsert,
} from './dbTypes';
import { DigitType, Job, NewJob, NewSystem, System, User } from '~/types';
import {
  and,
  arrayContains,
  asc,
  desc,
  eq,
  exists,
  getTableColumns,
  inArray,
  like,
  SQL,
  sql,
} from 'drizzle-orm';
import {
  systemFromDbEntity,
  dbInsertFromSystem,
  dbInsertFromJob,
  jobFromDbEntity,
} from './converters';

export async function getSystemById(
  id: number,
  userId: number,
): Promise<System | null> {
  const systems = await db
    .select({
      ...getTableColumns(systemsTable),
      digits: sql<number[][] | null>`
        (SELECT array_agg(${digitsTable.elements})
        FROM ${digitsTable} WHERE ${digitsTable}.${digitsTable.id} = ANY(${systemsTable}.${systemsTable.digitIds}))`.as(
        'digits',
      ),
      isFavourited: sql<boolean>`
        EXISTS (
          SELECT 1
          FROM ${favouritesTable}
          WHERE ${favouritesTable.systemId} = ${id}
          AND ${favouritesTable.userId} =  ${userId}
        )
        `.as('is_favourited'),
    })
    .from(systemsTable)
    .where(eq(systemsTable.id, id));
  if (systems.length == 0) {
    return null;
  }
  return systemFromDbEntity(systems[0]);
}

export type SystemsFilter = {
  dim?: number;
  name?: string;
  gns?: boolean;
  basePrefix?: number[];
  filterFavourites?: boolean;
  filterOwnedByUser?: boolean;
  digitType?: DigitType;
  digits?: number[][];
  page: number;
  pageSize: number;
};

function buildSystemFilters(filter: SystemsFilter, userId: number): SQL[] {
  const {
    dim,
    name,
    gns,
    basePrefix,
    filterFavourites,
    filterOwnedByUser,
    digitType,
    digits,
  } = filter;
  const filters: SQL[] = [];
  if (dim) {
    filters.push(eq(systemsTable.dimension, dim));
  }
  if (name) {
    filters.push(like(systemsTable.name, `%${name}%`));
  }
  if (gns != undefined) {
    filters.push(eq(systemsTable.isGNS, gns));
  }
  if (basePrefix && basePrefix.length != 0) {
    const basePrefixStr = basePrefix.join(',');
    filters.push(
      eq(
        sql`${systemsTable.base}[1:${sql.raw(basePrefix.length.toString())}]`,
        sql`ARRAY[${sql.raw(basePrefixStr)}]`,
      ),
    );
  }
  if (digitType) {
    filters.push(eq(systemsTable.digitType, digitType));
  }
  if (digitType == 'Explicit' && digits && digits.length != 0) {
    const subquery = db
      .select({ digitIds: sql<number[]>`array_agg(id)`.as('digit_ids') })
      .from(digitsTable)
      .where(inArray(digitsTable.elements, digits));
    filters.push(arrayContains(systemsTable.digitIds, subquery));
  }
  if (filterFavourites) {
    const subquery = db
      .select({ a: sql`1` })
      .from(favouritesTable)
      .where(
        and(
          eq(favouritesTable.systemId, systemsTable.id),
          eq(favouritesTable.userId, userId),
        ),
      );
    filters.push(exists(subquery));
  }
  if (filterOwnedByUser) {
    filters.push(eq(systemsTable.userId, userId));
  }
  return filters;
}

export async function getSystems(
  params: SystemsFilter,
  userId: number,
): Promise<{ systems: System[]; hasNext: boolean }> {
  const filters = buildSystemFilters(params, userId);
  const systems = await db
    .select({
      ...getTableColumns(systemsTable),
      digits: sql<number[][] | null>`
        (SELECT array_agg(${digitsTable.elements})
        FROM ${digitsTable} WHERE ${digitsTable.id} = ANY(${systemsTable.digitIds}))`.as(
        'digits',
      ),
      isFavourited: sql<boolean>`
        EXISTS (
          SELECT 1
          FROM ${favouritesTable}
          WHERE ${favouritesTable.systemId} = ${systemsTable.id}
          AND ${favouritesTable.userId} = ${userId}
        )
        `.as('is_favourited'),
    })
    .from(systemsTable)
    .leftJoin(favouritesTable, eq(systemsTable.id, favouritesTable.systemId))
    .where(and(...filters))
    .orderBy(
      desc(systemsTable.lastJob).append(sql` nulls last`),
      desc(systemsTable.id),
    )
    .limit(params.pageSize + 1)
    .offset((params.page - 1) * params.pageSize);

  const hasNext = systems.length > params.pageSize;
  if (hasNext) {
    systems.length = systems.length - 1;
  }
  return {
    systems: systems.map(systemFromDbEntity),
    hasNext,
  };
}

async function insertOrGetVectors(digits: number[][]): Promise<number[]> {
  const pgArrayString = digits
    .map((arr) => {
      return '(ARRAY[' + arr + '])';
    })
    .join(',');

  const res = await db.execute(
    sql.raw(`WITH input(elements) AS (
      VALUES ${pgArrayString}
    ),
    inserted AS (
        INSERT INTO "digits" (elements)
        SELECT elements FROM input
        ON CONFLICT (elements) DO NOTHING
        RETURNING *
    )
    (SELECT id FROM inserted

    UNION ALL

    SELECT id
    FROM "digits"
    WHERE "elements" IN (SELECT elements FROM input)
    AND NOT EXISTS (
        SELECT 1
        FROM inserted
    ));`),
  );
  const rows = res.rows as { id: number }[];
  return rows.map((res) => res.id);
}

export async function insertSystem(
  system: NewSystem,
  userId: number,
): Promise<System> {
  let digitIds: number[] | undefined;
  if (system.digits.type == 'Explicit') {
    digitIds = await insertOrGetVectors(system.digits.values);
  }
  const dbEntity: SystemDbInsert = dbInsertFromSystem(system, userId, digitIds);

  const res = (await db.insert(systemsTable).values(dbEntity).returning())[0];
  if (system.digits.type == 'Explicit') {
    return systemFromDbEntity({
      ...res,
      digits: system.digits.values,
      isFavourited: false,
    });
  }
  return systemFromDbEntity({ ...res, isFavourited: false });
}

export async function insertUser(user: UserDbInsert): Promise<User> {
  return (
    await db
      .insert(usersTable)
      .values(user)
      .returning({ id: usersTable.id, userName: usersTable.userName })
  )[0];
}

export async function getUserByUserName(
  userName: string,
): Promise<UserDbEntity | null> {
  const res = await db
    .select()
    .from(usersTable)
    .where(eq(usersTable.userName, userName));
  if (res.length == 0) {
    return null;
  }
  return res[0];
}

export async function getUserById(id: number) {
  return db.query.users.findFirst({
    where: (users, { eq }) => eq(users.id, id),
  });
}

export async function insertJob(job: NewJob) {
  let gridPointId: number | undefined;
  if (job.walkFrom) {
    gridPointId = (await insertOrGetVectors([job.walkFrom]))[0];
  }
  const dbEntity: JobDbInsert = dbInsertFromJob(job, gridPointId);

  const res = (await db.insert(jobsTable).values(dbEntity).returning())[0];
  if (job.walkFrom) {
    return jobFromDbEntity(res, job.walkFrom);
  }
  return jobFromDbEntity(res);
}

export type JobsFilter = {
  systemId?: number;
  userId?: number;
};

function buildJobFilters(params: JobsFilter): SQL[] {
  const { systemId, userId } = params;
  const filters: SQL[] = [];
  if (systemId) {
    filters.push(eq(jobsTable.systemId, systemId));
  }
  if (userId) {
    filters.push(eq(jobsTable.userId, userId));
  }
  return filters;
}

export async function getJobs(params: JobsFilter): Promise<Job[]> {
  const filters = buildJobFilters(params);
  const res = await db
    .select()
    .from(jobsTable)
    .where(and(...filters))
    .orderBy(desc(jobsTable.createdAt));

  return res.map((r) => jobFromDbEntity(r));
}

export async function favouriteSystem(systemId: number, userId: number) {
  const favourite: FavouriteDbInsert = { systemId, userId };
  await db.insert(favouritesTable).values(favourite);
}

export async function unFavouriteSystem(systemId: number, userId: number) {
  await db
    .delete(favouritesTable)
    .where(
      and(
        eq(favouritesTable.systemId, systemId),
        eq(favouritesTable.userId, userId),
      ),
    );
}
