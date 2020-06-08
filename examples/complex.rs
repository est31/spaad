#[spaad::entangled]
#[derive(Clone)]
struct X<T: 'static + Send + Clone, A>
where
    A: 'static + Send + Clone,
{
    t: T,
    a: A,
    b: i32,
}

#[spaad::entangled]
impl<T: 'static + Send + Clone, A> xtra::Actor for X<T, A>
where
    A: 'static + Send + Clone,
{
    fn started(&mut self, _: &mut xtra::Context<Self>) {}
}

#[spaad::entangled]
impl<T: 'static + Send + Clone, A> X<T, A>
where
    A: 'static + Send + Clone,
{
    #[spaad::spawn]
    fn new(t: T, a: A) -> X<T, A> {
        X { t, a, b: 0 }
    }

    #[spaad::handler]
    async fn foo(&mut self, mut h: f64, ctx: &mut xtra::Context<Self>) {
        self.b += 1;
        h += 1.0;
        println!("hello {}", h);
        println!("b = {}", self.as_ref()); // calling trait method on self
        self.blabla().await; // await needed - we are calling the async function itself.
        ctx.notify_immediately(Notification); // interop with normal xtra handlers
    }

    #[spaad::handler]
    async fn bar(&mut self) -> Result<(), xtra::Disconnected> {
        self.b -= 1;
        println!("goodbye");
        Ok(())
    }

    #[spaad::handler]
    async fn blabla(&mut self) {
        println!("middle!");
        self.not_a_handler().await;
    }

    async fn not_a_handler(&mut self) {
        println!("almost there!");
        self.not_async();
    }

    #[spaad::handler]
    fn not_async(&self) {
        println!("one more!!");
    }
}

struct Notification;
impl xtra::Message for Notification {
    type Result = ();
}

#[spaad::entangled]
#[async_trait::async_trait]
impl<T: 'static + Send + Clone, A> xtra::Handler<Notification> for X<T, A>
where
    A: 'static + Send + Clone,
{
    async fn handle(&mut self, _: Notification, ctx: &mut xtra::Context<Self>) {
        println!("stopping");
        ctx.stop();
    }
}

#[spaad::entangled]
impl<T: 'static + Send + Clone, A> AsRef<i32> for X<T, A>
where
    A: 'static + Send + Clone,
{
    fn as_ref(&self) -> &i32 {
        &self.b
    }
}

#[tokio::main]
async fn main() {
    #[allow(unused_mut)] // for intellij we set as mut :)
    let x = X::<u32, u32>::new(1, 2);
    x.foo(1.0).await;
    assert!(x.bar().await.is_err()); // disconnected, so we assert that it returned error
}
