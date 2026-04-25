import { Button } from '@kobalte/core/button';
import { Tabs } from '@kobalte/core/tabs';
import { useSubmission } from '@solidjs/router';
import { createEffect } from 'solid-js';
import { uploadSystemFromFileWithRedirect } from '~/api/queries';
import FileInput from '~/components/forms/FileInput';
import TextInput from '~/components/forms/TextInput';
import SystemForm from '~/components/systems/SystemForm';

const triggerClass: string =
  'inline-block px-2 py-4 outline-none hover:bg-highlight focus-visible:bg-highlight';

export default function NewSystem() {
  const submission = useSubmission(uploadSystemFromFileWithRedirect);

  return (
    <Tabs class="w-full text-foreground">
      <Tabs.List class="relative flex data-[orientation=horizontal]:items-center border-b border-b-faint">
        <Tabs.Trigger class={triggerClass} value="manual">
          Manual Input
        </Tabs.Trigger>
        <Tabs.Trigger class={triggerClass} value="file">
          Upload File
        </Tabs.Trigger>
        <Tabs.Indicator class="absolute bg-accent transition-all data-[orientation=horizontal]:h-0.5 data-[orientation=horizontal]:-bottom-px " />
      </Tabs.List>
      <Tabs.Content class="p-4" value="manual">
        <SystemForm />
      </Tabs.Content>
      <Tabs.Content class="p-4" value="file">
        <div class="flex items-center justify-center">
          <form
            class="flex flex-col min-w-md w-4/5 max-w-lg gap-2"
            method="post"
            enctype="multipart/form-data"
            action={uploadSystemFromFileWithRedirect}
          >
            <div class="text-red-500 h-4 w-full text-center ">
              {submission.error?.message}
            </div>
            <FileInput
              inputName="input-file"
              label={<span>Drop or select file</span>}
            />
            <Button class="bg-accent rounded-md px-4 py-2" type="submit">
              Submit
            </Button>
          </form>
        </div>
        {/* Add json structure sample */}
      </Tabs.Content>
    </Tabs>
  );
}
