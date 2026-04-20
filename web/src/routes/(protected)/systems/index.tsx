import GenericSystemsTable from '~/components/systems/GenericSystemsTable';

export default function Systems() {
  return <GenericSystemsTable initialFilters={{ page: 1, pageSize: 25 }} />;
}
