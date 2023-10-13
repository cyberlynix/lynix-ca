
<script>
    let time1 = '';
    let time2 = '';
    let timeDifference = '00:00:00';
  
    function calculateTimeDifference() {
      const time1Parts = time1.split(':');
      const time2Parts = time2.split(':');
  
      if (time1Parts.length === 3 && time2Parts.length === 3) {
        const hours1 = parseInt(time1Parts[0], 10);
        const minutes1 = parseInt(time1Parts[1], 10);
        const seconds1 = parseInt(time1Parts[2], 10);
        const hours2 = parseInt(time2Parts[0], 10);
        const minutes2 = parseInt(time2Parts[1], 10);
        const seconds2 = parseInt(time2Parts[2], 10);
  
        const totalSeconds1 = hours1 * 3600 + minutes1 * 60 + seconds1;
        const totalSeconds2 = hours2 * 3600 + minutes2 * 60 + seconds2;
  
        const timeDifferenceSeconds = totalSeconds1 - totalSeconds2;
        const hoursDifference = Math.floor(timeDifferenceSeconds / 3600);
        const minutesDifference = Math.floor((timeDifferenceSeconds % 3600) / 60);
        const secondsDifference = timeDifferenceSeconds % 60;
  
        timeDifference = `${hoursDifference.toString().padStart(2, '0')}:${minutesDifference.toString().padStart(2, '0')}:${secondsDifference.toString().padStart(2, '0')}`;
      } else {
        timeDifference = '00:00:00';
      }
    }
  </script>
  
<div class="min-h-screen p-5">
    <h1 class="text-5xl text-center text-cyan-400" style="text-shadow: 5px 5px #075985;">Time Calculator</h1>
    <p class="p-2 text-center">This is a tool to calculate the difference between 2 times with seconds.</p>
  
    <div class="text-center mt-4">
      <label for="time1">Time 1:</label>
      <input type="text" id="time1" bind:value={time1} placeholder="HH:MM:SS"  class="bg-black px-5 py-1.5 border border-gray-600" />
      <br/>
      <label for="time2">Time 2:</label>
      <input type="text" id="time2" bind:value={time2} placeholder="HH:MM:SS" class="bg-black px-5 py-1.5 border border-gray-600 mt-5"/>
      <br/>
      <button on:click={calculateTimeDifference} class="mt-5 bg-cyan-700 px-5 py-1.5 font-semibold">Calculate</button>
  
      <p class="mt-5">Time Difference: {timeDifference}</p>
    </div>
</div>
  
  
  
  