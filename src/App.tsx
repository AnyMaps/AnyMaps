import { Dialog, DialogTrigger, DialogContent, Button } from "@nipsys/shadcn-lsd";

function App() {
  return (
    <Dialog>
      <DialogTrigger>
        <Button>Open</Button>
      </DialogTrigger>
      <DialogContent>
        <p>Dialog content here</p>
      </DialogContent>
    </Dialog>
  );
}

export default App;
