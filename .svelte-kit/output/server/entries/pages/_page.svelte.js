import { c as create_ssr_component } from "../../chunks/ssr.js";
const app = "";
const Page = create_ssr_component(($$result, $$props, $$bindings, slots) => {
  return `<div class="text-center p-5 h-screen flex justify-center items-center" data-svelte-h="svelte-1gapqhs"><div><div class="flex justify-center"><img src="/images/lynix-cute.png" class="w-[200px] mb-5" alt="Lynix Cute"></div> <h1 class="text-4xl font-cyber text-cyan-600">Welcome to the <strong class="text-green-600">NEW</strong> lynix.ca!</h1> <p class="text-md mt-2 font-cyber text-gray-300">We&#39;re currently experiencing infrastructure upgrades. We&#39;ll be back soon!</p> <p class="text-sm font-cyber mt-3 text-gray-300"><strong><i>OwO - This page is using the new technology we&#39;re deploying soon!</i></strong></p></div></div>`;
});
export {
  Page as default
};
