import { ExplicitDigits, System } from '~/types';
import { columns, ColumnType, SystemColumn } from './systemTableColumns';
import Matrix from './entryFields/Matrix';
import VectorSet from './entryFields/VectorSet';

const generateCell = (system: System, columnType: ColumnType) => {
  switch (columnType) {
    case 'dimension':
      return (
        <span class="font-mono text-sm text-foreground">
          {system.dimension}
        </span>
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
      return system.signature === null ? (
        <span class="text-foreground text-sm">—</span>
      ) : (
        <span class="font-mono text-xs text-foreground">
          [{system.signature.join(', ')}]
        </span>
      );
    case 'lastJob':
      return system.lastJob === null ? (
        <span class="text-foreground text-sm">—</span>
      ) : (
        <span class="text-sm text-foreground">
          {system.lastJob.toLocaleString()}
        </span>
      );
    default:
      return null;
  }
};

export default function SystemEntry(props: { system: System }) {
  return (
    <tr class="border-b border-faint/50 hover:bg-highlight transition-colors">
      {columns.map((col) => (
        <td class="px-4 py-3 align-middle">
          {generateCell(props.system, col.type)}
        </td>
      ))}
    </tr>
  );
}
