pub fn posts() -> rust_fel::Element {
    let post_text = rust_fel::html(
        "<div>
          <h2>Posts</h2>
          <ul |id=post-titles|></ul>
          <script |src=./posts.js|></script>
        </div>"
            .to_owned(),
    );

    rust_fel::Element::new(
        "div".to_owned(),
        rust_fel::Props {
            class_name: Some("posts".to_owned()),
            children: Some(vec![post_text]),
            ..Default::default()
        },
    )
}
