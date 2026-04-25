import { createAsync, useParams } from '@solidjs/router';
import { JSX, Match, Show, Suspense, Switch } from 'solid-js';
import { getSystemById } from '~/api/server';
import Loading from '~/components/shared/Loading';
import { SystemContext } from '~/lib/SystemContext';

export default function SystemWrapper(props: { children: JSX.Element }) {
  const params = useParams();
  const systemId = parseInt(params.systemId!);
  const system = createAsync(() => getSystemById(systemId));

  return (
    <Suspense fallback={<Loading />}>
      <Switch fallback={<Loading />}>
        <Match when={system() == null}>
          <div class="flex items-center justify-center w-full h-full">
            <span>{`System with id [${params.systemId}] was not found`}</span>
          </div>
        </Match>
        <Match when={system()}>
          <SystemContext.Provider value={system}>
            <Suspense fallback={<Loading />}>{props.children}</Suspense>
          </SystemContext.Provider>
        </Match>
      </Switch>
    </Suspense>
  );
}
