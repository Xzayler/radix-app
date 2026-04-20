import { createAsync, useParams } from '@solidjs/router';
import { JSX, Match, Show, Suspense, Switch } from 'solid-js';
import { getSystemById } from '~/api/server';
import Loading from '~/components/shared/Loading';
import SystemDetails from '~/components/systems/SystemDetails';

export default function SystemPage(props: { children: JSX.Element }) {
  const params = useParams();
  const systemId = parseInt(params.systemId!);
  const system = createAsync(() => getSystemById(systemId));
  return (
    <Suspense fallback={<Loading />}>
      <Switch fallback={<Loading />}>
        <Match when={system() == null}>
          <div class="flex items-center justify-center w-full h-full">
            <span>{`System with id [${systemId}] was not found`}</span>
          </div>
        </Match>
        <Match when={system()}>
          <SystemDetails system={system()!} />
        </Match>
      </Switch>
    </Suspense>
  );
}
