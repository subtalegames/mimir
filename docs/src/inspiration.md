# Inspiration

Mímir is heavily inspired (both in concept and general architecture) by [Elan Ruskin's amazing session from GDC 2012 on AI-driven Dynamic Dialog][gdc]:

<div>
  <div style="position:relative;padding-top:56.25%;">
    <iframe src="https://www.youtube-nocookie.com/embed/tAbBID3N64A" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen
      style="position:absolute;top:0;left:0;width:100%;height:100%;"></iframe>
  </div>
</div>

Fundamentally speaking, Mímir is simply a Rust implementation of Elan's proposed system for dynamic dialogue.

However, Mímir does offer some differences and/or extensions that cater specifically to games developed internally at Subtale, as well as expanding in scope to include general purpose rule evaluation (not limited to dialogue).

[gdc]: https://www.youtube.com/watch?v=tAbBID3N64A