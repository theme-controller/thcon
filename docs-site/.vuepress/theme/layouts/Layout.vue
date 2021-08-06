<template>
  <Layout>
    <template #page-bottom>
      <div id="footer-container">
        <img src="/images/powered-by-vercel.svg" alt="Powered by Vercel"/>
      </div>
    </template>
  </Layout>
</template>

<script>
import Layout from '@vuepress/theme-default/lib/client/layouts/Layout.vue';

export default {
  components: {
    Layout,
  }
};
</script>

<style lang="css">
#footer-container {
  display: flex;
  justify-content: center;
}
</style>
