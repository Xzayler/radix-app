import { Tabs } from '@kobalte/core/tabs';
import { action, useSubmission } from '@solidjs/router';
import { createEffect, Show } from 'solid-js';
import { uploadSystem } from '~/api/server';
import FileInput from '~/components/shared/FileInput';

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
    <main>
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
          <form
            class="flex flex-col "
            method="post"
            enctype="multipart/form-data"
            action={uploadSystemAction}
          >
            <div>
              {'Error:' + submission.pending
                ? 'Pending'
                : (submission.error?.message ?? '')}
            </div>
            <FileInput
              inputName="input-file"
              errorMessage={
                submission.pending ? '' : (submission.error?.message ?? '')
              }
            />
            {/* TODO: Kobalte Button */}
            <button class="bg-accent rounded-md px-4 py-2" type="submit">
              Submit
            </button>
          </form>
          {/* Add json structure sample */}
        </Tabs.Content>
      </Tabs>
    </main>
  );
}
