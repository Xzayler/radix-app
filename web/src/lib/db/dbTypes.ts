import { InferInsertModel, InferSelectModel } from 'drizzle-orm';
import {
  digitsTable,
  favouritesTable,
  jobsTable,
  systemsTable,
  usersTable,
} from './schema';

export type UserDbEntity = InferSelectModel<typeof usersTable>;
export type UserDbInsert = InferInsertModel<typeof usersTable>;

export type SystemDbEntity = InferSelectModel<typeof systemsTable>;
export type SystemDbEntityWithDetails = SystemDbEntity & {
  digits?: number[][] | null;
  isFavourited: boolean;
};
export type SystemDbInsert = InferInsertModel<typeof systemsTable>;

export type DigitDbEntity = InferSelectModel<typeof digitsTable>;
export type DigitDbInsert = InferInsertModel<typeof digitsTable>;

export type FavouriteDbEntity = InferSelectModel<typeof favouritesTable>;
export type FavouriteDbInsert = InferInsertModel<typeof favouritesTable>;

export type JobDbEntity = InferSelectModel<typeof jobsTable> & {
  user?: UserDbEntity;
  input?: SystemDbEntity;
};
export type JobDbInsert = InferInsertModel<typeof jobsTable>;
