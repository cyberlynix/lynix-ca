<script lang="ts">
	import Test from '$lib/Test.svelte';
	import { onMount, onDestroy } from 'svelte';

	let timeLeft = 15 * 60; // 15 minutes in seconds
	let countdownInterval: any;

	interface Music {
		name: string;
		url: string;
		cover: string | null;
		artist: string | null;
	}

	// TODO: Load from api.lynix.ca (Soon, hardcoded for now)
	let music: Music[] = [
		{
			name: 'Have Some Fun',
			url: 'https://cdn.discordapp.com/attachments/538075938496184331/1143007954161913916/Have_Some_Fun.wav',
			cover: null,
			artist: 'asour'
		},
        {
            name: 'Do Some Parkour',
            url: 'https://cdn.discordapp.com/attachments/538075938496184331/1143007953650192456/Do_Some_Parkour.wav',
            cover: null,
            artist: 'asour'
        },
        {
            name: "Pylon Main 2.0.0",
            url: 'https://cdn.discordapp.com/attachments/538075938496184331/1143007954510024714/Pylon_Main_2.0.0.wav',
            cover: null,
            artist: 'asour'
        },
        {
            name: 'Deep Rumors',
            url: 'https://cdn.discordapp.com/attachments/538075938496184331/1143007954845585468/Deep_Rumors.wav',
            cover: null,
            artist: 'asour'
        }
	];
	function shuffle(array: any[]) {
		let currentIndex = array.length,
			randomIndex;

		// While there remain elements to shuffle.
		while (currentIndex != 0) {
			// Pick a remaining element.
			randomIndex = Math.floor(Math.random() * currentIndex);
			currentIndex--;

			// And swap it with the current element.
			[array[currentIndex], array[randomIndex]] = [array[randomIndex], array[currentIndex]];
		}

		return array;
	}
	function shuffleMusic() {
		shuffle(music);
	}
    shuffleMusic();
    let playingMusic: any = null; // audio element, but cant use that type globally or we get error
    let playingIndex = -1;
    let playingMusicInterface: Music = {
        name: 'Interact with the page to start the music!',
        artist: 'In OBS, there\'s a button to do that. Refresh page first if you still see this after.',
        cover: null,
        url: ''
    };

	onMount(() => {
        function playNextSong() {
            playingIndex ++;
            if (playingIndex >= music.length) {
                playingIndex = 0;
            }

            if (playingMusic !== null) {
                playingMusic.pause();
            }
            playingMusic = new Audio(music[playingIndex].url);
            playingMusic.play();
            playingMusic.volume = 0.1;
            playingMusic.playbackRate = 1.2;
            playingMusic.addEventListener('ended', () => {
                playNextSong();
            });
            playingMusicInterface = music[playingIndex];
        }
        playingMusicInterface = music[playingIndex];
        document.addEventListener('click', () => {
            playNextSong();
        });
        try {
            playNextSong();
        } catch (e) {
            console.log('error playing music');
        }

		function updateCountdown() {
			if (timeLeft <= 0) {
				clearInterval(countdownInterval);
			} else {
				timeLeft--;
				console.log('trying to load tick.mp3..');
				const tickSound = new Audio('/tick.mp3');
				let timeLeftEven = timeLeft % 2 == 0;
				let randomRange = (min: number, max: number) => {
					return Math.floor(Math.random() * (max - min + 1) + min);
				};
				let multiplier = randomRange(0.9, 1.1);
				if (timeLeftEven) {
					tickSound.playbackRate = 5 * multiplier;
					tickSound.volume = 0.1;
				} else {
					tickSound.playbackRate = 3 * multiplier;
					tickSound.volume = 0.12;
				}
				// faster

				try {
					tickSound.play();
				} catch (e) {}
			}
		}
		countdownInterval = setInterval(updateCountdown, 1000);
	});

	onDestroy(() => {
		clearInterval(countdownInterval);
        if(playingMusic) {
        playingMusic.pause();
        }
	});

	let timeFormatted = '';
	$: {
		const minutes = Math.floor(timeLeft / 60);
		const seconds = timeLeft % 60;
		timeFormatted = `${minutes}:${seconds.toString().padStart(2, '0')}`;
	}
</script>

<Test />
<div class="flex items-center justify-center w-full h-screen">
	<div class="w-full">
		<div>


            <!-- Song Box -->
            <div class="bg-dark shadow-lg flex items-center space-x-5 fixed top-[50px] left-5 p-5 pr-[200px]">
                <img src="/images/lynix-cute.png" alt="Lynix Cute" class="h-[75px] w-[75px] rounded-xl" />
                <div>
                    <h2 class="text-2xl font-semibold">{playingMusicInterface.name}</h2>
                    <p class="text-gray-400 text-xl">{playingMusicInterface.artist}</p>
                </div>
            </div>



			<div class="flex justify-center">
				<img src="/images/lynix-cute.png" class="h-[250px] w-[300px]" alt="lynix" />
			</div>
			<div class="bg-gradient-to-r from-cyan-700 to-green-700 py-5 w-full animated-gradient">
				<h1 class="text-3xl text-white font-cyber text-center w-full">
					<span class="text-white text-6xl"
						><span class="text-white">STARTING SOON</span><br />
						<span class="text-xl">The stream will start shortly, sit back and relax.</span><br
						/></span
					>
					<span class="text-4xl">{timeFormatted}</span>
				</h1>
			</div>
		</div>
	</div>
</div>
