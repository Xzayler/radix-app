import {jobsTable, usersTable} from './schema';

export type UserDbEntity = typeof usersTable.$inferSelect;

export type UserDbInsert =  typeof usersTable.$inferInsert;

export type JobDbInsert = typeof jobsTable.$inferInsert;