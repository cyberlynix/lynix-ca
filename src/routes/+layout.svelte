<script lang="ts">
    import '../app.css';
    import Navbar from "$lib/components/Navbar.svelte";
    import Alert from "$lib/components/Alert.svelte";
</script>

<div class="flex flex-col h-screen">
    <Navbar/>
    <Alert link="https://lynix.ca/articles/moving-serverless" linkName="Learn More" message="The lynix.ca has received a visual refresh and content changes!"/>
    
    <slot></slot>
</div>