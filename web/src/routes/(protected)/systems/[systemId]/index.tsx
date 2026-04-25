import { Match, Switch } from 'solid-js';
import Loading from '~/components/shared/Loading';
import SystemDetails from '~/components/systems/SystemDetails';
import { useSystemContext } from '~/lib/SystemContext';

export default function SystemPage() {
  const system = useSystemContext();

  return (
    <Switch fallback={<Loading />}>
      <Match when={system()}>
        <SystemDetails system={system()!} />
      </Match>
    </Switch>
  );
}
