<!-- src/App.svelte -->

<script>
    import { onMount } from 'svelte';
  
    let tabCount = 1;
    let tabs = [{ id: 1, time1: '', time2: '', timeDifference: '00:00:00', ffmpegCommand: '' }];
    let activeTab = 1;
    let concatCommands = '';
  
    function addTab() {
      tabCount++;
      const newTab = { id: tabCount, time1: '', time2: '', timeDifference: '00:00:00', ffmpegCommand: '' };
      tabs = [...tabs, newTab];
      setActiveTab(tabCount);
    }
  
    function setActiveTab(tabId) {
      activeTab = tabId;
    }
  
    function calculateTimeDifference(tab) {
      const time1Parts = tab.time1.split(':');
      const time2Parts = tab.time2.split(':');
  
      if (time1Parts.length === 3 && time2Parts.length === 3) {
        const hours1 = parseInt(time1Parts[0], 10);
        const minutes1 = parseInt(time1Parts[1], 10);
        const seconds1 = parseInt(time1Parts[2], 10);
        const hours2 = parseInt(time2Parts[0], 10);
        const minutes2 = parseInt(time2Parts[1], 10);
        const seconds2 = parseInt(time2Parts[2], 10);
  
        const totalSeconds1 = hours1 * 3600 + minutes1 * 60 + seconds1;
        const totalSeconds2 = hours2 * 3600 + minutes2 * 60 + seconds2;
  
        const timeDifferenceSeconds = totalSeconds2 - totalSeconds1;
        const hoursDifference = Math.floor(timeDifferenceSeconds / 3600);
        const minutesDifference = Math.floor((timeDifferenceSeconds % 3600) / 60);
        const secondsDifference = timeDifferenceSeconds % 60;
  
        tab.timeDifference = `${hoursDifference.toString().padStart(2, '0')}:${minutesDifference.toString().padStart(2, '0')}:${secondsDifference.toString().padStart(2, '0')}`;
  
        // Generate FFmpeg command
        tab.ffmpegCommand = `ffmpeg -i "2023-10-12 17-33-28.mkv" -ss ${tab.time1 ? tab.time1 : '00:00:00'} -t ${tab.timeDifference} -c copy -f mpegts ${tab.id.toString().padStart(2, '0')}.ts`;
        generateConcatCommands();
      } else {
        tab.timeDifference = '00:00:00';
        tab.ffmpegCommand = '';
      }
    }
  
    function generateConcatCommands() {
      const concatCommandsArray = tabs.map((tab) => `${tab.id.toString().padStart(2, '0')}.ts`);
      concatCommands = `ffmpeg -i "concat:${concatCommandsArray.join('|')}" -c copy "2023-10-12 17-33-28 (FINAL).mkv"`;
    }
  
    onMount(() => {
      calculateTimeDifference(tabs[0]);
    });
  </script>
  
  <div class="min-h-screen p-5">
    <h1 class="text-5xl text-center text-cyan-400" style="text-shadow: 5px 5px #075985;"><span class="text-red-500"  style="text-shadow: 5px 5px #611414;">Blood on the Clocktower</span> Video Splitter (BETA)</h1>
    <p class="p-2 text-center">This is a tool to calculate the difference between 2 times with seconds and generate FFmpeg commands for concatenation.</p>

    <div class="flex justify-center mt-5">
        <div class="w-3/4 border border-cyan-600 p-5">
            <div class="flex">
                <div class="w-[200px] border-r border-cyan-600">
                    
                    <div class="flex justify-center mr-5"><button on:click={addTab} class="bg-cyan-700 px-5 py-1.5 font-semibold">Add Tab</button></div>

                    <div class="flex flex-col mt-5 space-y-1 mr-5">
                    <!-- Tabs -->
                    {#each tabs as tab (tab.id)}
                        <button
                        class="{tab.id === activeTab ? 'bg-cyan-500' : 'bg-cyan-800'} bg-cyan-700 px-5 py-1.5 font-semibold"
                        on:click={() => setActiveTab(tab.id)}
                        >
                        Tab {tab.id}
                        </button>
                    {/each}
                    </div>
                </div>
                <div class="flex-1 p-5">
                    <h1 class="text-4xl">Tab {activeTab} - Options</h1>
                    {#each tabs as tab (tab.id)}
                    {#if tab.id === activeTab}
                      <label for="time1">Filename:</label>
                      <input type="text" bind:value={tab.time1} placeholder="Filename" class="bg-black px-5 py-1.5 border border-gray-600 mt-5" />
                      <br/>
                      <label for="time1">Time 1:</label>
                      <input type="text" bind:value={tab.time1} placeholder="HH:MM:SS" class="bg-black px-5 py-1.5 border border-gray-600 mt-5" />
                      <br/>
                      <label for="time2">Time 2:</label>
                      <input type="text" bind:value={tab.time2} placeholder="HH:MM:SS" class="bg-black px-5 py-1.5 border border-gray-600 mt-5"/>
                      <br/>
                      <button on:click={() => calculateTimeDifference(tab)} class="mt-5 bg-cyan-700 px-5 py-1.5 font-semibold">Calculate</button>
                      <div class="mt-5 flex justify-center">
                        <p>Time Difference: <span class="bg-black p-1">{tab.timeDifference}</span></p>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 text-cyan-500 ml-2">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
                        </svg>
                      </div>
                      <div class="mt-5 flex justify-center">
                        <p>Ffmpeg Command: <span class="bg-black p-1">{tab.ffmpegCommand}</span></p>
                        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6 text-cyan-500 ml-2">
                          <path stroke-linecap="round" stroke-linejoin="round" d="M15.666 3.888A2.25 2.25 0 0013.5 2.25h-3c-1.03 0-1.9.693-2.166 1.638m7.332 0c.055.194.084.4.084.612v0a.75.75 0 01-.75.75H9a.75.75 0 01-.75-.75v0c0-.212.03-.418.084-.612m7.332 0c.646.049 1.288.11 1.927.184 1.1.128 1.907 1.077 1.907 2.185V19.5a2.25 2.25 0 01-2.25 2.25H6.75A2.25 2.25 0 014.5 19.5V6.257c0-1.108.806-2.057 1.907-2.185a48.208 48.208 0 011.927-.184" />
                        </svg>
                      </div>
                    {/if}
                  {/each}
                </div>
            </div>
        </div>
    </div>
    
    
    <div class="mt-4">
      <p>Concatenation Command:</p>
      <p><span class="bg-black p-1">{concatCommands}</span></p>
    </div>
  </div>
  