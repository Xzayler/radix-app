import { InferInsertModel, InferSelectModel } from 'drizzle-orm';
import {jobsTable, systemsTable, usersTable} from './schema';
// import type { BuildQueryResult, DBQueryConfig, ExtractTablesWithRelations } from 'drizzle-orm';
// import { schema } from './schema';

// type TSchema = ExtractTablesWithRelations<typeof schema>;

// export type IncludeRelation<TableName extends keyof TSchema> = DBQueryConfig<
//   'one' | 'many',
//   boolean,
//   TSchema,
//   TSchema[TableName]
// >['with'];

// export type InferResultType<
//   TableName extends keyof TSchema,
//   With extends IncludeRelation<TableName> | undefined = undefined
// > = BuildQueryResult<
//   TSchema,
//   TSchema[TableName],
//   {
//     with: With;
//   }
// >;

// type Iftasd = InferResultType<'jobs', {input: true}>;

export type UserDbEntity = InferSelectModel<typeof usersTable>;

export type UserDbInsert =  InferInsertModel<typeof usersTable>;

export type SystemDbEntity = InferSelectModel<typeof systemsTable>;

export type SystemDbInsert = InferInsertModel<typeof systemsTable>;

export type JobDbEntity = InferSelectModel<typeof jobsTable> & {
  user?: UserDbEntity,
  input?: SystemDbEntity
};

export type JobDbInsert = InferInsertModel<typeof jobsTable>;