import { pgTable, integer, text, pgEnum, index, timestamp}  from "drizzle-orm/pg-core";

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

export const statusEnum = pgEnum('status', ['Pending', 'Running', 'Success', 'Failed']);

export const jobsTable = pgTable(
  'jobs',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userId: integer('user_id').references(() => usersTable.id).notNull(),
    status: statusEnum(),
    inputUri: text('input_uri').notNull(),
    outputUri: text('output_uri'),
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