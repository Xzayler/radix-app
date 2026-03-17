import { createAsync, useParams } from '@solidjs/router';
import { JSX, Match, Show, Suspense, Switch } from 'solid-js';
import { getSystemById } from '~/api/server';
import Loading from '~/components/shared/Loading';
import { SystemContext } from '~/lib/SystemContext';

export default function SystemPage(props: { children: JSX.Element }) {
  const params = useParams();
  const system = createAsync(() => getSystemById(parseInt(params.systemId!)));
  return (
    <Suspense fallback={<Loading />}>
      <Switch fallback={<Loading />}>
        <Match when={system() == null}>
          <div>
            <span>System with id {params.systemId} was not found</span>
          </div>
        </Match>
        <Match when={system()}>
          <SystemContext.Provider value={system}>
            <span>System: {system()?.id}</span>
            {props.children}
          </SystemContext.Provider>
        </Match>
      </Switch>
    </Suspense>
  );
}
