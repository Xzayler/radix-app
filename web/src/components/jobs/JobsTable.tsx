import { For } from 'solid-js';
import JobEntry from './JobEntry';
import { columns } from './jobTableColumns';
import { Job } from '~/types';

export default function JobsTable(props: { jobs: Job[] }) {
  return (
    <div class="rounded-lg bg-highlight bg-card">
      <div class="w-full overflow-x-auto">
        <table class="w-full">
          <thead>
            <tr class="">
              <For each={columns}>
                {(col) => (
                  <th class="px-4 py-3 text-left text-xs font-semibold uppercase tracking-wider text-foreground">
                    {col.label}
                  </th>
                )}
              </For>
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
