import { createAsync } from '@solidjs/router';
import { createEffect, createSignal, Suspense } from 'solid-js';
import { getSystems } from '~/api/server';
import { SystemsFilter } from '~/lib/db/operations';
import SystemsFilters from './SystemsFilters';
import SystemsTable from './SystemsTable';
import TablePagination from './TablePagination';
import Loading from '../shared/Loading';
import { System } from '~/types';

export default function GenericSystemsTable(props: {
  initialFilters: SystemsFilter;
}) {
  const [filters, setFilters] = createSignal(props.initialFilters);
  const listResponse = createAsync(
    () => {
      return getSystems(filters());
    },
    { initialValue: { systems: [], hasNext: false } },
  );

  createEffect(() => {
    setHasNext(listResponse().hasNext);
    setSystems(listResponse().systems);
  });

  let [hasNext, setHasNext] = createSignal<boolean>(false);
  let [systems, setSystems] = createSignal<System[]>([]);

  return (
    <div class="space-y-3 p-4">
      <div class="rounded-lg min-h-full space-y-3">
        <SystemsFilters value={filters()} setValue={setFilters} />{' '}
        <Suspense fallback={<Loading />}>
          <SystemsTable systems={systems()} />
        </Suspense>
      </div>
      <TablePagination
        value={filters().page}
        onChange={(p: number) => {
          const fs = { ...filters() };
          fs.page = p;
          setFilters(fs);
        }}
        hasNext={hasNext()}
      />
    </div>
  );
}
