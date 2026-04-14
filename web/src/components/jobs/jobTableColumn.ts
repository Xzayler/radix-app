export type ColumnType = 'user' | 'jobType' | 'norm' | 'status';

export type JobColumn = {
  type: ColumnType;
  label: string;
};

export const columns: JobColumn[] = [
  { type: 'user', label: 'Started By' },
  { type: 'jobType', label: 'Type' },
  { type: 'norm', label: 'Norm' },
  { type: 'status', label: 'Status' },
];
