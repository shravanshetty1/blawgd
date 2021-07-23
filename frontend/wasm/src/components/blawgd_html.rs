pub struct BlawgdHTMLDoc {
    page: Box<dyn super::Component>,
}

impl BlawgdHTMLDoc {
    pub fn new(page: Box<dyn super::Component>) -> Box<BlawgdHTMLDoc> {
        Box::new(BlawgdHTMLDoc { page })
    }
}

impl super::Component for BlawgdHTMLDoc {
    fn to_html(&self) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title> Blawgd </title>
    <link rel="stylesheet" href="style.css">
</head>
<body>
{}
</body>
<script type="module">
    import init from './pkg/client.js';

    init();
</script>
</html>
"#,
            self.page.to_html()
        )
    }
}
