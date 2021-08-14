<template>
  <Layout>
    <template #page-bottom>
      <div id="footer-container">
        <a href="https://vercel.com?utm_source=theme-controller&utm_campaign=oss">
          <img src="/images/powered-by-vercel.svg" alt="Powered by Vercel"/>
        </a>
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
