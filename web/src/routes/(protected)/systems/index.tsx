import { Button } from '@kobalte/core/button';
import { A } from '@solidjs/router';
import AddIcon from '~/components/shared/AddIcon';
import GenericSystemsTable from '~/components/systems/GenericSystemsTable';

export default function Systems() {
  return (
    <div class="space-y-3">
      <div class="flex gap-4 items-center">
        <h2 class="ml-4 text-lg font-semibold uppercase tracking-wider text-muted-foreground">
          Systems
        </h2>
        <Button class="rounded-md bg-accent cursor-pointer px-1 pr-2 hover:scale-105 transition-transform">
          <div class="">
            <A href="/systems/new" class="flex items-center">
              <div class="h-5 aspect-square">
                <AddIcon />
              </div>
              <span>New System</span>
            </A>
          </div>
        </Button>
      </div>
      <GenericSystemsTable initialFilters={{ page: 1, pageSize: 25 }} />
    </div>
  );
}
