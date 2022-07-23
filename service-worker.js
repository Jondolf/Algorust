const cacheName = 'algorust';
const filesToCache = [
  '/index.html',
  '/assets/images/GitHub-Mark-64px.png',
  '/assets/images/GitHub-Mark-Light-64px.png',
];

// Start the service worker and cache all of the app's content
self.addEventListener('install', function (e) {
  e.waitUntil(
    caches.open(cacheName).then(function (cache) {
      return cache.addAll(filesToCache);
    })
  );
});

// Serve cached content when offline
self.addEventListener('fetch', function (e) {
  e.respondWith(
    caches.match(e.request).then(function (response) {
      return response || fetch(e.request);
    })
  );
});
