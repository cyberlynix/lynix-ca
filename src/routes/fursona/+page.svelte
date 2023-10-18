
<script>
    import { onMount } from 'svelte';
  
    function render() {
        var scene = new THREE.Scene();
        var camera = new THREE.PerspectiveCamera(75, window.innerWidth / window.innerHeight, 0.1, 1000);
        var renderer = new THREE.WebGLRenderer();
        renderer.setSize(window.innerWidth, window.innerHeight);
        document.getElementById("container").appendChild(renderer.domElement);

        var loader = new THREE.FBXLoader();
        loader.load('your_model.fbx', function (object) {
            scene.add(object);
        });

        camera.position.z = 5;

        var animate = function () {
            requestAnimationFrame(animate);
            // Add animation or interaction logic here
            renderer.render(scene, camera);
        };

        animate();
    }

    onMount(() => {
        render();
    });
</script>

<svelte:head>
	<title>Lynix - Fursona</title>
</svelte:head>

<div class="min-h-screen px-10 py-10 font-cyber">
    <div class="mt-3 px-4 py-1 border border-red-600 bg-dark inline-flex items-center mx-2 sm:mx-5">
        <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width={1.5} stroke="currentColor" class="w-6 h-6 text-red-500 mr-2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M10.34 15.84c-.688-.06-1.386-.09-2.09-.09H7.5a4.5 4.5 0 110-9h.75c.704 0 1.402-.03 2.09-.09m0 9.18c.253.962.584 1.892.985 2.783.247.55.06 1.21-.463 1.511l-.657.38c-.551.318-1.26.117-1.527-.461a20.845 20.845 0 01-1.44-4.282m3.102.069a18.03 18.03 0 01-.59-4.59c0-1.586.205-3.124.59-4.59m0 9.18a23.848 23.848 0 018.835 2.535M10.34 6.66a23.847 23.847 0 008.835-2.535m0 0A23.74 23.74 0 0018.795 3m.38 1.125a23.91 23.91 0 011.014 5.395m-1.014 8.855c-.118.38-.245.754-.38 1.125m.38-1.125a23.91 23.91 0 001.014-5.395m0-3.46c.495.413.811 1.035.811 1.73 0 .695-.316 1.317-.811 1.73m0-3.46a24.347 24.347 0 010 3.46" />
        </svg>
        <span class="text-red-500"><strong>Attention!</strong> This section of the website may be outdated and not up to date with current references.</span>
    </div><br/>
    <div id="container"></div>
    <!--<div class="flex flex-wrap mt-3">
        <div class="w-full sm:w-1/2 md:w-1/2 lg:w-1/3 xl:w-1/3 p-5">
            <h1 class="text-4xl sm:text-5xl md:text-6xl ">My Fursona</h1>
            <p class="mt-3 text-base sm:text-md">
                <span class="text-cyan-500 text-md">What's a fursona?</span> A fursona
                is a customized animal character representing someone in the furry fandom,
                ranging from anthropomorphic personas to roleplay characters or digital
                mascots.
            </p>
            <p class="text-base sm:text-md mt-2">
                <span class="text-cyan-500 text-md">What's a furry?</span> A furry is
                someone who is a fan or enthusiast of characters that depict animals with
                human characteristics, such as talking, walking on two legs, and displaying
                human-like emotions and behaviors.
            </p>
            <h1 class="text-4xl sm:text-5xl md:text-6xl mt-5">About Lynix</h1>
            <p class="mt-3 text-base sm:text-lg">
                Lynix is a unique character known as a Freakhound, a canine hybrid with
                reptilian traits on its tail. Hailing from deep space, Lynix possesses
                advanced technological knowledge and aids in protecting Earth against cyber
                attacks. With striking dual-colored eyes (green and blue) and glowing
                details that respond to its environment and behavior, Lynix is equipped with
                an embedded implant that enables survival in various conditions, including
                the ability to breathe in space without oxygen. Supported by the Freak
                Network, Lynix instantly gains intelligence and detects threats.
                Additionally, Lynix can simulate real-world events and technological
                scenarios. Apart from its technological expertise, Lynix also takes joy in
                piloting spacecrafts and planes.
            </p>
        </div>
        <div class="w-full sm:w-1/2 md:w-1/2 lg:w-2/3 xl:w-2/3 p-5">
            <div class="w-full h-[550px] bg-dark mt-5 flex justify-center items-center text-center text-2xl">
                <p>
                    Reference Sheet Unavailable
                </p>
            </div>
            <div class="mt-3 px-4 py-1 bg-dark inline-block">
                Reference Sheet by <span class="text-cyan-500">Unknown</span>
            </div>
        </div>
    </div>-->
</div>