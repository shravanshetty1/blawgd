pub struct BlawgdHTMLDoc {
    page: Box<dyn super::Component>,
}

impl BlawgdHTMLDoc {
    pub fn new(page: Box<dyn super::Component>) -> Box<BlawgdHTMLDoc> {
        Box::new(BlawgdHTMLDoc { page })
    }
}

impl super::Component for BlawgdHTMLDoc {
    fn to_string(&self) -> String {
        format!(
            r#"
<!DOCTYPE html>
<html >
<head>
    <meta charset="UTF-8">
    <title> Samachar </title>
    <link rel="stylesheet" href="static/style.css">
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
            self.page.to_string()
        )
    }
}
