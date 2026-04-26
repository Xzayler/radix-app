import { A, createAsync } from '@solidjs/router';
import { getSystemJobs } from '~/api/server';
import JobsTable from './JobsTable';
import { Button } from '@kobalte/core/button';
import AddIcon from '../shared/AddIcon';
import { createSignal } from 'solid-js';
import NewSystemJob from './NewSystemJob';

export default function SystemJobsTable(props: {
  systemId: number;
  dimension: number;
}) {
  const jobs = createAsync(() => getSystemJobs(props.systemId), {
    initialValue: [],
  });

  return (
    <div class="space-y-3">
      <div class="flex gap-6 items-center">
        <Button class="">
          <A
            class="text-md font-bold uppercase hover:underline"
            href={`/systems/${props.systemId}`}
          >
            {'< Back'}
          </A>
        </Button>
        <h2 class="text-lg font-semibold uppercase tracking-wider text-muted-foreground">
          Analyses
        </h2>
      </div>
      <div class="overflow-y-hidden">
        <NewSystemJob systemId={props.systemId} dimension={props.dimension} />
      </div>
      <JobsTable jobs={jobs()} />
    </div>
  );
}
