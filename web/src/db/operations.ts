
import { db } from "./db";
import { usersTable, jobsTable } from "./schema";
import { JobDbInsert, UserDbEntity, UserDbInsert } from "./dbTypes";
import { User } from "~/types";


function insertJob(job: JobDbInsert) {
  db.insert(jobsTable).values(job);
}

export async function insertUser(user: UserDbInsert): Promise<User> {
  return (await db.insert(usersTable).values(user).returning(
    {id: usersTable.id, userName: usersTable.userName}))[0];
}

export async function getByUserName(userName: string) : Promise<UserDbEntity | undefined> {
  return db.query.users.findFirst({
    where: (users, {eq}) => eq(users.userName, userName),
  });
}

export async function getUserById(id: number) {
  return db.query.users.findFirst({
    where: (users, {eq}) => eq(users.id, id),
  })
}
