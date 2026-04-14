import { Button } from '@kobalte/core/button';
import { action, createAsync, useParams, useSubmission } from '@solidjs/router';
import { createEffect, createSignal, Show, Suspense } from 'solid-js';
import { getSystemById, queueJob } from '~/api/server';
import StyledSelect from '~/components/forms/StyledSelect';
import VectorInput from '~/components/forms/VectorInput';
import Loading from '~/components/shared/Loading';
import { useSystemContext } from '~/lib/SystemContext';
import { JobType, Norm } from '~/types';

export default function NewSystemJob() {
  const queueJobAction = action(queueJob);
  const queueJobSubmission = useSubmission(queueJobAction);
  createEffect(() => {
    if (queueJobSubmission.pending) {
      console.log('Pending Submission...');
    } else {
      console.log('Not pending');
      console.log(queueJobSubmission.error);
      console.log(queueJobSubmission.result);
    }
  });

  const system = useSystemContext();
  const [typeValue, setTypeValue] = createSignal<JobType | undefined>(
    'Decision',
  );

  return (
    <main>
      <Suspense fallback={<Loading />}>
        <div class="flex gap-2 items-stretch">
          <form
            class="flex flex-col grow"
            method="post"
            enctype="multipart/form-data"
            action={queueJobAction}
          >
            <div>
              <Show
                when={!queueJobSubmission.pending && queueJobSubmission.error}
              >
                <span>{queueJobSubmission.error}</span>
              </Show>
            </div>
            <input
              type="text"
              name="system-id"
              value={system()!.id}
              class="hidden"
            />
            <StyledSelect<Norm>
              label="Norm"
              name="norm"
              options={['Infinite', 'L1', 'L2']}
              defaultValue={'Infinite'}
            />
            <StyledSelect<JobType>
              label="JobType"
              name="job-type"
              placeholder="Type"
              options={['Walk', 'Decision', 'Classification']}
              value={typeValue()}
              onChange={setTypeValue}
            />
            <Show when={system()} fallback={<Loading />}>
              <></>
              <VectorInput
                label="Start point"
                name="grid-point"
                placeholder="0 1 2 ..."
                dim={system()!.dimension}
                disabled={typeValue() != 'Walk'}
              />
            </Show>
            <Button
              class="bg-accent rounded-md px-4 py-2 mr-auto mt-1"
              type="submit"
            >
              Submit
            </Button>
          </form>
          <div class="border-x border-ui rounded-full"></div>
          <div class="grow"></div>
        </div>
      </Suspense>
    </main>
  );
}
