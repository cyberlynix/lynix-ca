<script lang="ts">
    import '../app.css';
    import Navbar from "$lib/components/Navbar.svelte";
    import Alert from "$lib/components/Alert.svelte";
</script>

<div class="flex flex-col h-screen">
    <Navbar/>

    <slot></slot>

    <script src="https://cdnjs.cloudflare.com/ajax/libs/three.js/r128/three.min.js"></script>
    <script src="https://threejs.org/examples/js/loaders/FBXLoader.js"></script>
</div>