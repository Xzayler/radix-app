import { Job } from '~/types';
import { columns, ColumnType } from './jobTableColumns';
import { createSignal } from 'solid-js';
import { Button } from '@kobalte/core/button';
import { A, createAsync } from '@solidjs/router';
import { getDownloadUrl } from '~/api/server';
import DownloadIcon from '../shared/DownloadIcon';

const generateCell = (
  job: Job,
  columnType: ColumnType,
  downloadUrl?: string,
) => {
  switch (columnType) {
    case 'jobType':
      return (
        <span class="font-mono text-sm text-foreground">{job.jobType}</span>
      );
    case 'norm':
      return <span class="font-mono text-sm text-foreground">{job.norm}</span>;
    case 'status':
      return (
        <span class="font-mono text-sm text-foreground">{job.status}</span>
      );
    case 'startedAt':
      return job.startedAt ? (
        <span class="text-sm text-foreground">
          {job.startedAt.toLocaleString()}
        </span>
      ) : (
        <span class="text-foreground text-sm">—</span>
      );
    case 'finishedAt':
      return job.finishedAt ? (
        <span class="text-sm text-foreground">
          {job.finishedAt.toLocaleString()}
        </span>
      ) : (
        <span class="text-foreground text-sm">—</span>
      );
    case 'output':
      return (
        <Button
          class="text-accent aspect-square h-8 cursor-pointer disabled:text-faint disabled:cursor-not-allowed "
          disabled={downloadUrl === undefined}
        >
          {downloadUrl ? (
            <A href={downloadUrl}>
              <DownloadIcon />
            </A>
          ) : (
            <DownloadIcon />
          )}
        </Button>
      );
  }
};

export default function JobEntry(props: { job: Job }) {
  const downloadUrl = props.job.outputUri
    ? createAsync(() => getDownloadUrl(props.job.outputUri!))
    : undefined;
  return (
    <tr class="border-b border-faint/50 hover:bg-highlight transition-colors">
      {columns.map((col) => (
        <td class="px-4 py-3 align-middle">
          {generateCell(props.job, col.type)}
        </td>
      ))}
    </tr>
  );
}
