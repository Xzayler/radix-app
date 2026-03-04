'use server';
import { db } from './db';
import { usersTable, systemsTable } from './schema';
import { SystemDbInsert, UserDbEntity, UserDbInsert } from './dbTypes';
import { User } from '~/types';
import { eq } from 'drizzle-orm';

export async function insertSystem(system: SystemDbInsert) {
  await db.insert(systemsTable).values(system);
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
