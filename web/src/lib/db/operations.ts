'use server';
import { db } from './db';
import { usersTable, systemsTable, digitsTable, jobsTable } from './schema';
import {
  JobDbInsert,
  SystemDbInsert,
  UserDbEntity,
  UserDbInsert,
} from './dbTypes';
import { Job, NewJob, System, User } from '~/types';
import {
  and,
  arrayContains,
  asc,
  eq,
  getTableColumns,
  inArray,
  SQL,
  sql,
} from 'drizzle-orm';
import {
  systemFromDbEntity,
  dbInsertFromSystem,
  dbInsertFromJob,
  jobFromDbEntity,
} from './converters';

export type SystemsFilter = {
  base?: number[];
  digits?: number[][];
  orderBy?: {
    field: 'id' | 'lastJob';
    asc: boolean;
  };
};

export async function getSystemById(id: number): Promise<System | null> {
  const systems = await db
    .select({
      ...getTableColumns(systemsTable),
      digits: sql<number[][] | null>`
        (SELECT array_agg(${digitsTable.elements})
        FROM ${digitsTable} WHERE ${digitsTable}.${digitsTable.id} = ANY(${systemsTable}.${systemsTable.digitIds}))`.as(
        'digits',
      ),
    })
    .from(systemsTable)
    .where(eq(systemsTable.id, id));
  if (systems.length == 0) {
    return null;
  }
  return systemFromDbEntity(systems[0]);
}

function buildSystemFilters(filter: SystemsFilter): SQL[] {
  const { digits } = filter;
  const filters: SQL[] = [];
  if (digits) {
    const subquery = db
      .select({ digitIds: sql<number[]>`array_agg(id)`.as('digit_ids') })
      .from(digitsTable)
      .where(and(inArray(digitsTable.elements, digits)));
    filters.push(arrayContains(systemsTable.digitIds, subquery));
  }
  return filters;
}

export async function getSystems(params: SystemsFilter): Promise<System[]> {
  const filters = buildSystemFilters(params);
  const systems = await db
    .select({
      ...getTableColumns(systemsTable),
      digits: sql<number[][] | null>`
        (SELECT array_agg(${digitsTable.elements})
        FROM ${digitsTable} WHERE ${digitsTable}.${digitsTable.id} = ANY(${systemsTable}.${systemsTable.digitIds}))`.as(
        'digits',
      ),
    })
    .from(systemsTable)
    .where(and(...filters));

  return systems.map(systemFromDbEntity);
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
  system: Omit<System, 'id'>,
): Promise<System> {
  let digitIds: number[] | undefined;
  if (system.digits.type == 'Explicit') {
    digitIds = await insertOrGetVectors(system.digits.values);
  }
  const dbEntity: SystemDbInsert = dbInsertFromSystem(system, digitIds);

  const res = (await db.insert(systemsTable).values(dbEntity).returning())[0];
  if (system.digits.type == 'Explicit') {
    return systemFromDbEntity({ ...res, digits: system.digits.values });
  }
  return systemFromDbEntity(res);
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
    .orderBy(asc(jobsTable.createdAt));

  return res.map((r) => jobFromDbEntity(r));
}
