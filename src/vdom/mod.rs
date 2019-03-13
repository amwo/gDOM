macro_rules! render {
    ($html:expr) => {
        println!("{}", $html);
    };
}

struct Vdom();

impl Vdom {
    pub fn init() -> {
    }

    pub fn get_before_state -> {
    }

    pub fn get_after_state -> {
    }

    pub fn diff() -> {
    }

    pub fn patch() -> {
    }
}

#[test]
fn test_render() {
    render!(
        r#"
            <div>
                <h1></h1>
                <div>hoge</div>
                <button>Click</button>
            </div>
        "#
    );
}
