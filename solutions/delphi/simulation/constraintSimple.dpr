program constraintSimple;

uses
  System.StartUpCopy,
  FMX.Forms,
  constraintSimple.fmain in 'constraintSimple.fmain.pas' {FMain},
  ConstraintParticle in '..\src\ConstraintParticle.pas',
  ConstraintParticle.Renderer.FMX2D in '..\src\ConstraintParticle.Renderer.FMX2D.pas',
  ConstraintParticle.Simulation.BasicDistance in '..\src\ConstraintParticle.Simulation.BasicDistance.pas',
  ConstraintParticle.Simulation.DistanceChain in '..\src\ConstraintParticle.Simulation.DistanceChain.pas',
  ConstraintParticle.Simulation.SeparateCollision in '..\src\ConstraintParticle.Simulation.SeparateCollision.pas';

{$R *.res}

begin
  {$IFDEF DEBUG}
  ReportMemoryLeaksOnShutdown := true;
  {$ENDIF}
  Application.Initialize;
  Application.CreateForm(TFMain, FMain);
  Application.Run;
end.
