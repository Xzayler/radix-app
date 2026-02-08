import { pgTable, integer, text, pgEnum, index, timestamp, boolean}  from "drizzle-orm/pg-core";

export const usersTable = pgTable(
  'users',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userName: text('user_name').unique().notNull(),
    password: text('password').notNull()
  },
  (table) => [
      index('user_name_index').on(table.userName)
  ]
);

export const statusEnum = pgEnum('status', 
  ['Pending', 'Running', 'Success', 'Failed']);
export const jobTypeEnum = pgEnum('job_type', ['Walk', 'Decision', 'Classification']);
export const inputTypeEnum = pgEnum('input_type',
  ['Custom', 'Canonical', 'J-Canonical', 'Dense', 'Adjoined', 'Symmetric', 'Shifted']);

export const jobsTable = pgTable(
  'jobs',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userId: integer('user_id').references(() => usersTable.id).notNull(),
    status: statusEnum('status').notNull(),
    jobType: jobTypeEnum('job_type').notNull(),
    inputType: inputTypeEnum('input_type').notNull(),
    inputUri: text('input_uri').notNull(),
    outputUri: text('output_uri'),
    isNumberSystem: boolean('is_number_system'),
    createdAt: timestamp('created_at', {mode: "date", precision: 0, withTimezone: true}).defaultNow(),
    startedAt: timestamp('started_at', {mode: "date", precision: 0, withTimezone: true}),
    finishedAt: timestamp('finished_at', {mode: "date", precision: 0, withTimezone: true})
  },
  (table) => [
      index('user_id_index').on(table.userId)
    ]
)

export const schema = {
  users: usersTable,
  jobs: jobsTable
}