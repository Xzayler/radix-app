import { Tabs } from '@kobalte/core/tabs';
import { action, useSubmission } from '@solidjs/router';
import { createEffect } from 'solid-js';
import { uploadSystem } from '~/api/server';
import FileInput from '~/components/forms/FileInput';
import TextInput from '~/components/forms/TextInput';

const triggerClass: string =
  'inline-block px-2 py-4 outline-none hover:bg-highlight focus-visible:bg-highlight';

export default function NewSystem() {
  const uploadSystemAction = action(uploadSystem);
  const submission = useSubmission(uploadSystemAction);
  createEffect(() => {
    if (submission.pending) {
      console.log('Pending Submission...');
    } else {
      console.log('Not pending');
      console.log(submission);
      console.log(submission.error);
      console.log(submission.result);
    }
  });

  return (
    <Tabs class="w-full text-foreground">
      <Tabs.List class="relative flex data-[orientation=horizontal]:items-center border-b border-b-faint">
        <Tabs.Trigger class={triggerClass} value="file">
          File
        </Tabs.Trigger>
        <Tabs.Trigger class={triggerClass} value="manual">
          Manual
        </Tabs.Trigger>
        <Tabs.Indicator class="absolute bg-accent transition-all data-[orientation=horizontal]:h-0.5 data-[orientation=horizontal]:-bottom-px " />
      </Tabs.List>
      <Tabs.Content class="p-4" value="manual">
        Manual Input
      </Tabs.Content>
      <Tabs.Content class="p-4" value="file">
        <div class="flex items-center justify-center">
          <form
            class="flex flex-col min-w-md w-4/5 max-w-lg gap-2"
            method="post"
            enctype="multipart/form-data"
            action={uploadSystemAction}
          >
            <div>
              {' ' +
                (submission.pending
                  ? 'Uploading'
                  : (submission.error?.message ?? ''))}
            </div>
            <div>
              <TextInput
                name="name"
                label={<span>System Name:</span>}
                placeholder="System name"
                maxLength={32}
                required={true}
              />
            </div>
            <FileInput
              inputName="input-file"
              errorMessage={
                submission.pending ? '' : (submission.error?.message ?? '')
              }
              label={<span>Drop or select file</span>}
            />
            {/* TODO: Kobalte Button */}
            <button class="bg-accent rounded-md px-4 py-2" type="submit">
              Submit
            </button>
          </form>
        </div>
        {/* Add json structure sample */}
      </Tabs.Content>
    </Tabs>
  );
}
