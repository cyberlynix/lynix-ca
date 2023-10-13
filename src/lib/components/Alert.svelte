<script lang="ts">
	export let message: string;
    export let link: string; // Do you need linkName? for inner <a> tag?
    export let linkName: string;
</script>

<!-- Example Usage: <Alert link="https://lynix.ca/carroted-is-awesome" linkName="See More" type="error", msg=""/> -->

<!-- Colors for each alert -->
<!-- DEFCON: from-purple-700 to-blue-700 
     NORMAL-INFO: from-green-600 to to-blue-600
     WARNING-ERROR: from-red-600 to to-orange-600
--> 

<div class="w-full px-4 py-1 inline-flex items-center justify-center w-full bg-gradient-to-r from-cyan-700 to-blue-700 py-5 w-full animated-gradient">
    <span class="text-white text-xl text-center">
        <!-- Alert Message -->
        {message}
    </span>
</div>