export type ColumnType =
  | 'user'
  | 'jobType'
  | 'norm'
  | 'status'
  | 'output'
  | 'startedAt'
  | 'finishedAt'
  | 'error';

export type JobColumn = {
  type: ColumnType;
  label: string;
};

export const columns: JobColumn[] = [
  { type: 'jobType', label: 'Type' },
  { type: 'norm', label: 'Norm' },
  { type: 'status', label: 'Status' },
  { type: 'error', label: 'Error' },
  { type: 'startedAt', label: 'Started' },
  { type: 'finishedAt', label: 'Finished' },
  { type: 'output', label: 'Output' },
];
