// vite.config.ts
import { sveltekit } from "file:///home/lynix/Documents/lynix-ca/node_modules/@sveltejs/kit/src/exports/vite/index.js";
import { defineConfig } from "file:///home/lynix/Documents/lynix-ca/node_modules/vite/dist/node/index.js";
import { imagetools } from "file:///home/lynix/Documents/lynix-ca/node_modules/vite-imagetools/dist/index.js";
var vite_config_default = defineConfig({
  plugins: [
    imagetools({
      defaultDirectives: new URLSearchParams({
        format: "webp",
        as: "picture"
      })
    }),
    sveltekit()
  ]
});
export {
  vite_config_default as default
};
//# sourceMappingURL=data:application/json;base64,ewogICJ2ZXJzaW9uIjogMywKICAic291cmNlcyI6IFsidml0ZS5jb25maWcudHMiXSwKICAic291cmNlc0NvbnRlbnQiOiBbImNvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9kaXJuYW1lID0gXCIvaG9tZS9seW5peC9Eb2N1bWVudHMvbHluaXgtY2FcIjtjb25zdCBfX3ZpdGVfaW5qZWN0ZWRfb3JpZ2luYWxfZmlsZW5hbWUgPSBcIi9ob21lL2x5bml4L0RvY3VtZW50cy9seW5peC1jYS92aXRlLmNvbmZpZy50c1wiO2NvbnN0IF9fdml0ZV9pbmplY3RlZF9vcmlnaW5hbF9pbXBvcnRfbWV0YV91cmwgPSBcImZpbGU6Ly8vaG9tZS9seW5peC9Eb2N1bWVudHMvbHluaXgtY2Evdml0ZS5jb25maWcudHNcIjtpbXBvcnQgeyBzdmVsdGVraXQgfSBmcm9tICdAc3ZlbHRlanMva2l0L3ZpdGUnO1xuaW1wb3J0IHsgZGVmaW5lQ29uZmlnIH0gZnJvbSAndml0ZSc7XG5pbXBvcnQgeyBpbWFnZXRvb2xzIH0gZnJvbSAndml0ZS1pbWFnZXRvb2xzJztcblxuZXhwb3J0IGRlZmF1bHQgZGVmaW5lQ29uZmlnKHtcblx0cGx1Z2luczogW1xuXHRcdGltYWdldG9vbHMoe1xuXHRcdFx0ZGVmYXVsdERpcmVjdGl2ZXM6IG5ldyBVUkxTZWFyY2hQYXJhbXMoe1xuXHRcdFx0XHRmb3JtYXQ6ICd3ZWJwJyxcblx0XHRcdFx0YXM6ICdwaWN0dXJlJ1xuXHRcdFx0fSlcblx0XHR9KSxcblx0XHRzdmVsdGVraXQoKVxuXHRdXG59KTtcbiJdLAogICJtYXBwaW5ncyI6ICI7QUFBNFEsU0FBUyxpQkFBaUI7QUFDdFMsU0FBUyxvQkFBb0I7QUFDN0IsU0FBUyxrQkFBa0I7QUFFM0IsSUFBTyxzQkFBUSxhQUFhO0FBQUEsRUFDM0IsU0FBUztBQUFBLElBQ1IsV0FBVztBQUFBLE1BQ1YsbUJBQW1CLElBQUksZ0JBQWdCO0FBQUEsUUFDdEMsUUFBUTtBQUFBLFFBQ1IsSUFBSTtBQUFBLE1BQ0wsQ0FBQztBQUFBLElBQ0YsQ0FBQztBQUFBLElBQ0QsVUFBVTtBQUFBLEVBQ1g7QUFDRCxDQUFDOyIsCiAgIm5hbWVzIjogW10KfQo=
