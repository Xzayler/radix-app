import { Button } from '@kobalte/core/button';
import { action, useSubmission } from '@solidjs/router';
import { createEffect, createSignal, Show, Suspense } from 'solid-js';
import { queueJob } from '~/api/server';
import StyledSelect from '~/components/forms/StyledSelect';
import VectorInput from '~/components/forms/VectorInput';
import { JobType, Norm } from '~/types';

export default function NewSystemJob(props: {
  systemId: number;
  dimension: number;
}) {
  const queueJobAction = action(queueJob);
  const queueJobSubmission = useSubmission(queueJobAction);

  const [typeValue, setTypeValue] = createSignal<JobType>('Decision');

  return (
    <div class="flex gap-2 items-stretch">
      <form
        class="flex flex-col grow"
        method="post"
        enctype="multipart/form-data"
        action={queueJobAction}
      >
        <div>
          <div class="text-red-500">{queueJobSubmission.error?.message}</div>
        </div>
        <input
          type="text"
          name="system-id"
          value={props.systemId}
          class="hidden"
        />
        <div class="flex items-center gap-x-3">
          <StyledSelect<Norm>
            label="Norm"
            name="norm"
            options={['Infinite', 'L1', 'L2']}
            defaultValue={'Infinite'}
          />
          <StyledSelect<JobType>
            label="Analysis Type"
            name="job-type"
            placeholder="Type"
            options={['Path', 'Decision', 'Classification']}
            value={typeValue()}
            onChange={setTypeValue}
          />
          <VectorInput
            label="Start point"
            name="grid-point"
            placeholder="0 1 2 ..."
            dim={props.dimension}
            disabled={typeValue() != 'Path'}
          />
          <Button
            class="bg-accent rounded-md px-4 py-2 ml-auto hover:opacity-90 cursor-pointer"
            type="submit"
          >
            Start
          </Button>
        </div>
      </form>
    </div>
  );
}
