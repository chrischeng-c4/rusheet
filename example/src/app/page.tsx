import Grid from '@/components/grid/Grid';

export default function Home() {
  return (
    <main className="h-screen w-screen flex flex-col">
      <header className="h-12 bg-gray-800 text-white flex items-center px-4">
        <h1 className="text-lg font-semibold">RuSheet - Rust WASM Spreadsheet</h1>
      </header>
      <div className="flex-1">
        <Grid />
      </div>
    </main>
  );
}
