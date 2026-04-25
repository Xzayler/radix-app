import { createAsync } from '@solidjs/router';
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
  const [openState, setOpenState] = createSignal<boolean>(false);

  const jobs = createAsync(() => getSystemJobs(props.systemId), {
    initialValue: [],
  });

  const toggleOpenState = () => {
    setOpenState(!openState());
  };

  return (
    <div class="space-y-3">
      <div class="flex justify-between items-center">
        <h2 class="text-lg font-semibold uppercase tracking-wider text-muted-foreground">
          Jobs
        </h2>
        <Button
          onClick={toggleOpenState}
          class="rounded-md bg-accent cursor-pointer px-1 pr-2 hover:scale-105 transition-transform"
        >
          <div class="flex">
            <div class="h-5 aspect-square">
              <AddIcon />
            </div>
            <span>New Analysis</span>
          </div>
        </Button>
      </div>
      <div
        classList={{ 'h-auto': openState(), 'h-0': !openState() }}
        class="overflow-y-hidden transition-all ease-in-out duration-500 h-0"
      >
        <NewSystemJob systemId={props.systemId} dimension={props.dimension} />
      </div>
      <JobsTable jobs={jobs()} />
    </div>
  );
}
