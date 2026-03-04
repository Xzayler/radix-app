import { relations } from 'drizzle-orm';
import {
  pgTable,
  integer,
  text,
  pgEnum,
  index,
  timestamp,
  boolean,
} from 'drizzle-orm/pg-core';

export const usersTable = pgTable(
  'users',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userName: text('user_name').unique().notNull(),
    password: text('password').notNull(),
  },
  (table) => [index('user_name_index').on(table.userName)],
);

export const digitTypeEnum = pgEnum('input_type', [
  'Explicit',
  'Canonical',
  'J-Canonical',
  'Dense',
  'Adjoined',
  'Symmetric',
  'Shifted',
]);

export const systemsTable = pgTable(
  'systems',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    dimension: integer('dimension').notNull(),
    base: integer('base').array().notNull(),
    digitType: digitTypeEnum('digit_type').notNull(),
    isGNS: boolean('is_gns'),
    signature: integer('signature').array(),
    inputUri: text('input_uri').notNull(),
  },
  (table) => [
    // index('system')
  ],
);

export const statusEnum = pgEnum('status', [
  'Pending',
  'Running',
  'Success',
  'Failed',
]);
export const jobTypeEnum = pgEnum('job_type', [
  'Walk',
  'Decision',
  'Classification',
]);

export const jobsTable = pgTable(
  'jobs',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userId: integer('user_id')
      .references(() => usersTable.id)
      .notNull(),
    systemId: integer('system_id')
      .references(() => systemsTable.id)
      .notNull(),
    status: statusEnum('status').notNull(),
    jobType: jobTypeEnum('job_type').notNull(),
    outputUri: text('output_uri'),
    createdAt: timestamp('created_at', {
      mode: 'date',
      precision: 0,
      withTimezone: true,
    }).defaultNow(),
    startedAt: timestamp('started_at', {
      mode: 'date',
      precision: 0,
      withTimezone: true,
    }),
    finishedAt: timestamp('finished_at', {
      mode: 'date',
      precision: 0,
      withTimezone: true,
    }),
  },
  (table) => [
    index('system_id_index').on(table.systemId),
    index('user_id_index').on(table.userId),
  ],
);

export const jobsRelations = relations(jobsTable, ({ one }) => ({
  input: one(systemsTable, {
    fields: [jobsTable.systemId],
    references: [systemsTable.id],
  }),
  user: one(usersTable, {
    fields: [jobsTable.userId],
    references: [usersTable.id],
  }),
}));

export const schema = {
  users: usersTable,
  systems: systemsTable,
  jobs: jobsTable,
  jobsRelations,
};
