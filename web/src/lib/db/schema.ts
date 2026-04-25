import { relations } from 'drizzle-orm';
import {
  pgTable,
  integer,
  text,
  pgEnum,
  index,
  timestamp,
  boolean,
  primaryKey,
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

export const digitTypeEnum = pgEnum('digit_type', [
  'Explicit',
  'Canonical',
  'JCanonical',
  'Adjoined',
  'Symmetric',
  'JSymmetric',
  'Shifted',
]);

export const systemsTable = pgTable(
  'systems',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    name: text('name').notNull(),
    userId: integer('user_id').references(() => usersTable.id, {
      onDelete: 'set null',
    }),
    dimension: integer('dimension').notNull(),
    base: integer('base').array().notNull(),
    digitType: digitTypeEnum('digit_type').notNull(),
    isGNS: boolean('is_gns'),
    signature: integer('signature').array(),
    lastJob: timestamp('last_job', {
      mode: 'date',
      precision: 0,
      withTimezone: true,
    }),
    // Digit-specific fields
    digitIds: integer('digits').array(), // array of ids in vector table
    digitParam: integer('digit_param'),
  },
  // (table) => [index('digits_index').using('gin', table.digitIds)],
);

export const digitsTable = pgTable('digits', {
  id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
  elements: integer('elements').array().notNull().unique(),
});

export const favouritesTable = pgTable(
  'favourites',
  {
    userId: integer('user_id')
      .references(() => usersTable.id, {
        onDelete: 'cascade',
      })
      .notNull(),
    systemId: integer('system_id')
      .references(() => systemsTable.id, {
        onDelete: 'cascade',
      })
      .notNull(),
  },
  (table) => [primaryKey({ columns: [table.userId, table.systemId] })],
);

export const userRelations = relations(usersTable, ({ many }) => ({
  favourites: many(favouritesTable),
}));

export const systemsRelations = relations(systemsTable, ({ many }) => ({
  favouritedBy: many(usersTable),
}));

export const favouritesRelations = relations(favouritesTable, ({ one }) => ({
  user: one(usersTable, {
    fields: [favouritesTable.userId],
    references: [usersTable.id],
  }),
  system: one(systemsTable, {
    fields: [favouritesTable.systemId],
    references: [systemsTable.id],
  }),
}));

export const statusEnum = pgEnum('status', [
  'Pending',
  'Running',
  'Succeeded',
  'Failed',
]);
export const jobTypeEnum = pgEnum('job_type', [
  'Walk',
  'Decision',
  'Classification',
]);
export const normEnum = pgEnum('norm_type', ['Infinite', 'L1', 'L2']);

export const jobsTable = pgTable(
  'jobs',
  {
    id: integer('id').primaryKey().generatedAlwaysAsIdentity(),
    userId: integer('user_id')
      .references(() => usersTable.id)
      .notNull(),
    systemId: integer('system_id')
      .references(() => systemsTable.id, { onDelete: 'cascade' })
      .notNull(),
    status: statusEnum('status').notNull(),
    jobType: jobTypeEnum('job_type').notNull(),
    norm: normEnum('norm').notNull(),
    walkFrom: integer('walk_from'),
    outputUri: text('output_uri'),
    createdAt: timestamp('created_at', {
      mode: 'date',
      precision: 0,
      withTimezone: true,
    })
      .defaultNow()
      .notNull(),
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
    error: text('error'),
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
  userRelations,
  systems: systemsTable,
  systemsRelations,
  digits: digitsTable,
  favourites: favouritesTable,
  favouritesRelations,
  jobs: jobsTable,
  jobsRelations,
};
