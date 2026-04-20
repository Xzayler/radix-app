import { For } from 'solid-js';
import JobEntry from './JobEntry';
import { columns } from './jobTableColumns';
import { Job } from '~/types';

export default function JobsTable(props: { jobs: Job[] }) {
  return (
    <div class="rounded-lg border border-faint bg-card">
      <div class="w-full overflow-x-auto">
        <table class="w-full border-collapse">
          <thead>
            <tr class="border-b border-faint">
              {columns.map((col) => (
                <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-foreground">
                  {col.label}
                </th>
              ))}
            </tr>
          </thead>
          <tbody>
            <For each={props.jobs}>{(job) => <JobEntry job={job} />}</For>
          </tbody>
        </table>
      </div>
    </div>
  );
}
