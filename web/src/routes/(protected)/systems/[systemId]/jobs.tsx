import { useParams } from '@solidjs/router';
import { Show, Suspense } from 'solid-js';
import SystemJobsTable from '~/components/jobs/SystemJobsTable';
import Loading from '~/components/shared/Loading';
import { useSystemContext } from '~/lib/SystemContext';

export default function SystemJobs() {
  const system = useSystemContext();
  return (
    <div class="mx-auto max-w-6xl space-y-8">
      <Suspense fallback={<Loading />}>
        <Show when={system()}>
          <SystemJobsTable
            systemId={system()!.id}
            dimension={system()!.dimension}
          />
        </Show>
      </Suspense>
    </div>
  );
}
