import { createEffect, createSignal, For } from 'solid-js';
import { System } from '~/types';
import { columns, ColumnType, SystemColumn } from './systemTableColumns';
import Matrix from './entryFields/Matrix';
import VectorSet from './entryFields/VectorSet';
import { favourite, unFavourite } from '~/api/server';
import StarIcon from '../shared/StarIcon';
import { A, redirect, useNavigate } from '@solidjs/router';

const generateCell = (
  system: System,
  columnType: ColumnType,
  isFavourited: () => boolean,
  toggleFavourite: () => Promise<void>,
) => {
  switch (columnType) {
    case 'dimension':
      return (
        <span class="font-mono text-sm text-foreground">
          {system.dimension}
        </span>
      );
    case 'name':
      return (
        <div class="overflow-x-scroll max-w-28">
          <span class="text-foreground text-sm text-wrap wrap-break-word">
            {system.name}
          </span>
        </div>
      );
    case 'base':
      return <Matrix matrix={system.base} />;
    case 'digitType':
      return <span class="text-foreground text-sm">{system.digits.type}</span>;
    case 'digits':
      if (system.digits.type == 'Explicit')
        return <VectorSet vectors={system.digits.values} />;
      return <span></span>;
    case 'isGns':
      return system.isGns === null ? (
        <span class="text-foreground text-sm">—</span>
      ) : (
        <span
          class={`text-sm font-medium ${system.isGns ? 'text-green-600' : 'text-red-500'}`}
        >
          {system.isGns ? 'Yes' : 'No'}
        </span>
      );
    case 'signature':
      if (!system.signature) {
        return <span class="text-foreground text-sm">—</span>;
      }
      return (
        <div class="overflow-y-scroll max-h-52">
          <span class="font-mono text-xs text-foreground">
            [{system.signature.join(', ')}]
          </span>
        </div>
      );
    case 'lastJob':
      return system.lastJob === null ? (
        <span class="text-foreground text-sm">—</span>
      ) : (
        <span class="text-sm text-foreground">
          {system.lastJob.toLocaleString()}
        </span>
      );
    case 'operations':
      return (
        <div
          class={
            'h-6 aspect-square cursor-pointer transition-colors hover:text-yellow-500 ' +
            (isFavourited() ? 'text-yellow-500' : 'text-faint')
          }
          onClick={(e) => {
            toggleFavourite();
            e.stopPropagation();
          }}
        >
          <StarIcon toFill={isFavourited()} />
        </div>
      );
  }
};

export default function SystemEntry(props: { system: System }) {
  const [isFavourited, setIsFavourited] = createSignal(
    props.system.isFavourited,
  );

  createEffect(() => {
    setIsFavourited(props.system.isFavourited);
  });

  const toggleFavourite = async () => {
    const nextValue = !isFavourited();
    setIsFavourited(nextValue);

    try {
      if (nextValue) {
        await favourite(props.system.id);
      } else {
        await unFavourite(props.system.id);
      }
    } catch (error) {
      setIsFavourited(!nextValue);
      console.error('Failed to update favourite status', error);
    }
  };

  const navigate = useNavigate();

  return (
    <tr
      onClick={(e) => {
        navigate(`/systems/${props.system.id}`);
      }}
      class="border-b border-faint/50 hover:bg-highlight transition-colors cursor-pointer"
    >
      <For each={columns}>
        {(col) => (
          <td class="px-4 py-3 align-middle">
            {generateCell(
              props.system,
              col.type,
              isFavourited,
              toggleFavourite,
            )}
          </td>
        )}
      </For>
    </tr>
  );
}
