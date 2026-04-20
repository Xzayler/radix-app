import { Job } from '~/types';
import { columns, ColumnType } from './jobTableColumns';
import { Accessor, createSignal, For } from 'solid-js';
import { Button } from '@kobalte/core/button';
import { A, createAsync } from '@solidjs/router';
import { getDownloadUrl } from '~/api/server';
import DownloadIcon from '../shared/DownloadIcon';

const generateCell = (
  job: Job,
  columnType: ColumnType,
  downloadUrl: string | null,
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
        <span class="text-foreground text-sm">-</span>
      );
    case 'finishedAt':
      return job.finishedAt ? (
        <span class="text-sm text-foreground">
          {job.finishedAt.toLocaleString()}
        </span>
      ) : (
        <span class="text-foreground text-sm">-</span>
      );
    case 'output':
      return (
        <Button
          class="text-accent aspect-square h-8 cursor-pointer disabled:text-faint disabled:cursor-not-allowed "
          disabled={downloadUrl === null}
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
    case 'error':
      return (
        <div class="overflow-y-scroll max-h-52 max-w-64">
          <span class="">{job.error ?? ''}</span>
        </div>
      );
  }
};

export default function JobEntry(props: { job: Job }) {
  const downloadUrl = createAsync<string | null>(
    async () => {
      if (!props.job.outputUri) {
        return null;
      }
      const a = await getDownloadUrl(props.job.outputUri!);
      return a;
    },
    {
      initialValue: null,
    },
  );
  return (
    <tr class="border-b border-faint/50 hover:bg-highlight transition-colors">
      <For each={columns}>
        {(col) => (
          <td class="px-4 py-3 align-middle">
            {generateCell(props.job, col.type, downloadUrl())}
          </td>
        )}
      </For>
    </tr>
  );
}
