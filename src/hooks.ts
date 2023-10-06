export const handle = async ({ event, resolve }: any) => {
    const response = await resolve(event, {
      ssr: false,
    });
    return response;
};