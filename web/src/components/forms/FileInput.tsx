import { FileField } from '@kobalte/core/file-field';
import { JSX } from 'solid-js';

export default function FileInput(props: {
  inputName: string;
  errorMessage?: string;
  label: JSX.Element;
}) {
  return (
    <FileField
      class="flex flex-col items-center content-center w-full "
      accept={['.json']}
      allowDragAndDrop={true}
      onFileAccept={(data) => console.log('accepted:', data)}
      onFileReject={(data) => console.log('rejected: ', data)}
    >
      <FileField.Dropzone class="flex flex-col items-center gap-y-1.5 justify-center rounded-xl border-solid border border-ui w-full py-16">
        <div>{props.label}</div>
        <FileField.Trigger class="bg-accent rounded-md px-4 py-2 ">
          Browse file
        </FileField.Trigger>
      </FileField.Dropzone>
      <FileField.HiddenInput name={props.inputName} />
      <FileField.ItemList class="w-4/5 mt-2">
        {(file) => (
          <FileField.Item class="flex justify-between items-center rounded-xl border border-solid border-ui w-full max-w-80 p-4 mx-auto">
            {/* <FileField.ItemPreviewImage /> */}
            <FileField.ItemName />
            {/* <FileField.ItemSize /> */}
            <FileField.ItemDeleteTrigger class="bg-red-900 rounded-md px-4 py-2 ">
              Delete
            </FileField.ItemDeleteTrigger>
          </FileField.Item>
        )}
      </FileField.ItemList>
      <FileField.Description />
      <FileField.ErrorMessage>{props.errorMessage}</FileField.ErrorMessage>
    </FileField>
  );
}
